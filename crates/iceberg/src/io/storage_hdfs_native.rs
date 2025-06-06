use std::collections::HashMap;

use opendal::services::HdfsNativeConfig;
use opendal::{Configurator, Operator};

use crate::{Error, ErrorKind, Result};

/// hdfs clsuter name
pub const HDFS_NAMENODE: &str = "fs.defaultFS";

pub(crate) fn hdfs_native_config_parse(m: HashMap<String, String>) -> Result<HdfsNativeConfig> {
    let mut cfg = HdfsNativeConfig::default();

    let cluster_name = if let Some(cluster_name) = m.get(HDFS_NAMENODE) {
        cluster_name.to_string()
    } else {
        return Err(
            Error::new(ErrorKind::Unexpected, "hdfs.host is required to use HDFS")
                .with_context("config", format!("{m:?}")),
        );
    };

    cfg.name_node = Some(cluster_name);
    cfg.props = m;

    Ok(cfg)
}

/// Build new opendal operator from give path.
pub(crate) fn hdfs_native_config_build(cfg: &HdfsNativeConfig) -> Result<Operator> {
    let builder = cfg.clone().into_builder();
    Ok(Operator::new(builder)?.finish())
}
