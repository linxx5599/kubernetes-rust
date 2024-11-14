// use kube::config::Kubeconfig;
use kube::{
    config::{KubeConfigOptions, Kubeconfig},
    Config,
};
use kube_client::Client;

pub struct MKubeClient {}

impl MKubeClient {
    pub async fn new() -> Result<Client, Box<dyn std::error::Error>> {
        //读取dockerfile 环境变量CLUSTER_PATH
        let kubeconfig_path = std::env::var("CLUSTER_PATH").unwrap_or("src/k3s.yaml".to_string());
        let kubeconfig = Kubeconfig::read_from(kubeconfig_path).unwrap();
        let config = Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;
        let client = Client::try_from(config)?;
        Ok(client)
    }
}
