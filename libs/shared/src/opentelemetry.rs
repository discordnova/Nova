use std::ops::{Deref, DerefMut};

use opentelemetry_otlp::{ExportConfig, Protocol};
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct ExportConfigDeserialize(ExportConfig);
impl Clone for ExportConfigDeserialize {
    fn clone(&self) -> Self {
        Self(ExportConfig {
            endpoint: self.0.endpoint.clone(),
            protocol: self.0.protocol,
            timeout: self.0.timeout,
        })
    }
}

impl From<ExportConfigDeserialize> for ExportConfig {
    fn from(val: ExportConfigDeserialize) -> Self {
        val.0
    }
}

impl Deref for ExportConfigDeserialize {
    type Target = ExportConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExportConfigDeserialize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for ExportConfigDeserialize {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Fields {
            Endpoint,
            Timeout,
        }

        struct OpenTelemetryExportConfigVisitor;
        impl<'de> Visitor<'de> for OpenTelemetryExportConfigVisitor {
            type Value = ExportConfigDeserialize;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct OpenTelemetryExportConfig")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut export_config = ExportConfigDeserialize::default();
                export_config.0.protocol = Protocol::Grpc;
                while let Some(name) = map.next_key::<Fields>()? {
                    match name {
                        Fields::Endpoint => {
                            export_config.0.endpoint = map.next_value()?;
                        }
                        Fields::Timeout => {
                            export_config.0.timeout = map.next_value()?;
                        }
                    }
                }

                Ok(export_config)
            }
        }

        deserializer.deserialize_struct(
            "OpenTelemetryExportConfig",
            &["endpoint", "protocol", "timeout"],
            OpenTelemetryExportConfigVisitor,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub traces: Option<ExportConfigDeserialize>,
    pub metrics: Option<ExportConfigDeserialize>,
}
