use tokio::sync::mpsc;
use crate::models::ShiftData;

pub async fn flush_to_disk(batch: &[ShiftData]) {
    println!("💾 Flushed batch of {} records to disk.", batch.len());
    
    // TODO: Implement actual File I/O, Database Insert, or Parquet writing here.
}

pub async fn start_background_worker(mut rx: mpsc::Receiver<ShiftData>) {
    let mut buffer: Vec<ShiftData> = Vec::with_capacity(500);
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if !buffer.is_empty() {
                    println!("⏱️ Timeout reached. Flushing...");
                    flush_to_disk(&buffer).await;
                    buffer.clear();
                }
            }
            msg = rx.recv() => {
                match msg {
                    Some(payload) => {
                        buffer.push(payload);
                        println!("📦 Payload received. Current buffer size: {}", buffer.len());
                        if buffer.len() >= 500 {
                            println!("📦 Batch full. Flushing...");
                            flush_to_disk(&buffer).await;
                            buffer.clear();
                            interval.reset(); 
                        }
                    }
                    None => {
                        println!("Channel closed. Flushing remaining {} items.", buffer.len());
                        if !buffer.is_empty() {
                            flush_to_disk(&buffer).await;
                        }
                        break;
                    }
                }
            }
        }
    }
}
