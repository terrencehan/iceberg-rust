use std::collections::HashMap;

use opendal::services::HdfsConfig;
use opendal::{Configurator, Operator};

use crate::{Error, ErrorKind, Result};

/// hdfs clsuter name
pub const HDFS_NAMENODE: &str = "hdfs.namenode";

pub(crate) fn hdfs_config_parse(mut m: HashMap<String, String>) -> Result<HdfsConfig> {
    let mut cfg = HdfsConfig::default();

    let cluster_name = if let Some(cluster_name) = m.remove(HDFS_NAMENODE) {
        cluster_name
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
pub(crate) fn hdfs_config_build(cfg: &HdfsConfig) -> Result<Operator> {
    let builder = cfg.clone().into_builder();
    Ok(Operator::new(builder)?.finish())
}
