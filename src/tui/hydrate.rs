use crate::models::events_metrics::EventsMetrics;
use crate::models::materialized_views::events_by_type::EventsByType;
use crate::tui::app::{App, DeserializedNode};
use charybdis::operations::Find;
use charybdis::options::Consistency::One;
use chrono::{Timelike, Utc};
use futures::TryStreamExt;
use scylla::query::Query;
use scylla::CachingSession;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub fn start_hydration(db_app: &Arc<Mutex<App>>, db: Arc<CachingSession>) {
    let app = Arc::clone(db_app);
    let db = Arc::clone(&db);
    tokio::spawn(async move {
        loop {
            hydrate_events_stream(&app, &db).await;
            hydrate_driver_data(&app, &db).await;
            hydrate_listed_events(&app, &db).await;

            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    });
}

async fn hydrate_listed_events(app: &Arc<Mutex<App>>, db: &Arc<CachingSession>) {
    let listened_events = EventsMetrics::find_all()
        .consistency(One)
        .execute(&db)
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    let mut app = app.lock().await;
    if listened_events.is_empty() {
        app.listened_events = vec![EventsMetrics::default()];
    }


    app.listened_events = listened_events;
}

async fn hydrate_driver_data(app: &Arc<Mutex<App>>, db: &Arc<CachingSession>) {
    let mut app = app.lock().await;
    let db = db.get_session();
    let metrics = db.get_metrics();
    let cluster = db.get_cluster_data();

    app.metrics.update(metrics);
    app.nodes = DeserializedNode::transform_nodes(cluster.get_nodes_info());
}

async fn hydrate_events_stream(app: &Arc<Mutex<App>>, db: &Arc<CachingSession>) {
    let mut app = app.lock().await;
    let selected_item = &app.listened_events[app.selected_event.clone()];

    let current_timestamp = Utc::now()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    let query =
        Query::new("SELECT * FROM events_by_type WHERE event_type = ? AND bucket_id = ? LIMIT 100");

    let mut events_result = db
        .execute_iter(
            query,
            (selected_item.clone().event_type.as_str(), current_timestamp),
        )
        .await
        .unwrap()
        .rows_stream::<EventsByType>()
        .unwrap();

    let mut events = vec![];

    while let Some(event) = events_result.try_next().await.unwrap() {
        events.push(event);
    }

    app.recent_events = events;
}
