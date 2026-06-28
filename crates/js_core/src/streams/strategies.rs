use crate::js;

#[derive(Debug, Clone)]
pub enum QueuingStrategy {
    Count { high_water_mark: usize },
    ByteLength { high_water_mark: usize },
}

impl QueuingStrategy {
    pub fn high_water_mark(&self) -> usize {
        match self {
            QueuingStrategy::Count { high_water_mark }
            | QueuingStrategy::ByteLength { high_water_mark } => *high_water_mark,
        }
    }

    pub fn chunk_size(&self, chunk: &js::Value<'_>) -> usize {
        match self {
            QueuingStrategy::Count { .. } => 1,
            QueuingStrategy::ByteLength { .. } => chunk
                .as_object()
                .and_then(|obj| obj.get::<_, usize>("length").ok())
                .unwrap_or(1),
        }
    }

    pub fn from_js_object(strategy: &js::Object<'_>) -> js::Result<Self> {
        let high_water_mark: usize = strategy.get("highWaterMark").unwrap_or(1);
        let has_size_fn: bool = strategy.get::<_, js::Function>("size").is_ok();

        if has_size_fn {
            Ok(QueuingStrategy::ByteLength { high_water_mark })
        } else {
            Ok(QueuingStrategy::Count { high_water_mark })
        }
    }
}

impl Default for QueuingStrategy {
    fn default() -> Self {
        QueuingStrategy::Count { high_water_mark: 1 }
    }
}
