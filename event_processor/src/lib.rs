pub mod evento;
pub mod payloads;
pub mod pipeline;
pub mod traits;

// Re-exportar os items mais usados na raiz
pub use evento::Evento;
pub use payloads::{AlertaPayload, LogPayload, MetricaPayload};
pub use traits::{Alertavel, Formatavel, Processavel};
