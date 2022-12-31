// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use hyper::StatusCode;
use passivized_docker_engine_client::model::MountMode::ReadOnly;
use passivized_docker_engine_client::requests::{CreateContainerRequest, HostConfig};
use passivized_docker_engine_client::DockerEngineClient;
use tempfile::NamedTempFile;

use super::errors::TestError;
use super::images;
use super::resources::{set_config_permissions, write_default_conf};

pub struct IpfsContainer {
    docker: DockerEngineClient,
    container_id: String,
    pub ip: String,
}

impl IpfsContainer {
    pub async fn new(name: &str, image_name: &str, image_tag: &str) -> Result<Self, TestError> {
        let docker = DockerEngineClient::new()?;

        docker
            .images()
            .pull_if_not_present(image_name, image_tag)
            .await?;

        let create_request = CreateContainerRequest::default()
            .name(name)
            .image(format!("{}:{}", image_name, image_tag))
            .cmd(vec!["daemon", "--migrate=true"]);

        let container = docker.containers().create(create_request).await?;

        docker.container(&container.id).start().await?;

        let inspected = docker.container(&container.id).inspect().await?;

        let ip = inspected.first_ip_address().unwrap().to_string();

        let result = Self {
            docker,
            container_id: container.id,
            ip,
        };

        Ok(result)
    }

    pub async fn teardown(&self) -> Result<(), TestError> {
        self.docker.container(&self.container_id).stop().await?;

        self.docker.container(&self.container_id).remove().await?;

        Ok(())
    }
}

pub struct NginxContainer {
    /// While not used after assignment, we hold it in scope so it drops when Self drops.
    _default_conf: NamedTempFile,
    _htpasswd: NamedTempFile,
    docker: DockerEngineClient,
    container_id: String,
    pub ip: String,
    pub username: String,
    pub password: String,
}

impl NginxContainer {
    pub async fn new(name: &str, target_ip: &str) -> Result<Self, TestError> {
        const PROXY_USERNAME: &str = "foo";
        const PROXY_PASSWORD: &str = "bar";

        let default_conf = NamedTempFile::new()?;
        let default_conf_file = default_conf.path().to_str().unwrap();

        write_default_conf(target_ip, default_conf.path()).await?;

        let htpasswd_path = NamedTempFile::new()?;

        {
            let mut htpasswd = passivized_htpasswd::Htpasswd::new();
            htpasswd.set(PROXY_USERNAME, PROXY_PASSWORD).unwrap();
            htpasswd.write_to_path(&htpasswd_path).unwrap();
        }

        set_config_permissions(htpasswd_path.path()).unwrap();

        let htpasswd_file = htpasswd_path.path().to_str().unwrap().to_string();

        let docker = DockerEngineClient::new()?;

        docker
            .images()
            .pull_if_not_present(images::nginx::IMAGE, images::nginx::TAG)
            .await?;

        let create_request = CreateContainerRequest::default()
            .name(name)
            .image(format!("{}:{}", images::nginx::IMAGE, images::nginx::TAG))
            .host_config(
                HostConfig::default()
                    .auto_remove()
                    .mount(
                        default_conf_file,
                        "/etc/nginx/conf.d/default.conf",
                        ReadOnly,
                    )
                    .mount(htpasswd_file, "/secrets/htpasswd", ReadOnly),
            );

        let container = docker.containers().create(create_request).await?;

        docker.container(&container.id).start().await?;

        let inspected = docker.container(&container.id).inspect().await?;

        let ip = inspected.first_ip_address().unwrap().to_string();

        let result = Self {
            _default_conf: default_conf,
            _htpasswd: htpasswd_path,
            docker,
            container_id: container.id,
            ip,
            username: PROXY_USERNAME.into(),
            password: PROXY_PASSWORD.into(),
        };

        Ok(result)
    }

    pub async fn wait_for(&self) -> Result<(), String> {
        use std::time::Duration;

        cfg_if::cfg_if! {
            if #[cfg(feature = "with-actix")] {
                const MESSAGE: &str = "Failed to connect to host: Connection refused";
            } else if #[cfg(feature = "with-hyper")] {
                const MESSAGE: &str = "tcp connect error";
            }
        }

        let url = format!("http://{}", self.ip);

        let mut attempts = 0;

        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let failure = match reqwest::get(&url).await {
                Ok(response) => {
                    if response.status() == StatusCode::UNAUTHORIZED {
                        // Successfully connected, and because we did not pass any credentials,
                        // we expect an Unauthorized response.
                        None
                    } else {
                        Some(format!("HTTP status {}", response.status()))
                    }
                }
                Err(e) => Some(format!("{:?}", e)),
            };

            match failure {
                None => break,
                Some(msg) => {
                    eprintln!("Not ready yet: {}", msg);

                    if msg.contains(MESSAGE) {
                        attempts += 1;

                        if attempts >= 7 {
                            return Err(format!("Already tried {} times", attempts));
                        }
                    } else {
                        return Err(format!("Other failure: {}", msg));
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn teardown(&self) -> Result<(), TestError> {
        self.docker.container(&self.container_id).stop().await?;

        Ok(())
    }
}
