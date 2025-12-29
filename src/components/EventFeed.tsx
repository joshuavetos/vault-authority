import { VaultEvent } from "../App";

export default function EventFeed({ events }: { events: VaultEvent[] }) {
  return (
    <section className="panel event-feed">
      <h2>CHRONOLOGICAL TRUTH LOG</h2>
      <div className="log-container">
        {events.map((ev, i) => (
          <div key={i} className={`log-entry ${ev.type}`}>
            <span className="time">[{ev.timestamp}]</span>
            <span className="type">{ev.type.toUpperCase()}</span>
            <span className="data">{JSON.stringify(ev.data)}</span>
          </div>
        ))}
      </div>
    </section>
  );
}
