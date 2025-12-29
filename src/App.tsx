import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import EventFeed from "./components/EventFeed";
import StatusPanel from "./components/StatusPanel";

export type VaultEvent = {
  type: "RemediationAttempted" | "RemediationExecuted" | "RemediationCommitted" | "ReceiptGenerated" | "RemediationRefused";
  data: any;
  timestamp: string;
};

export default function App() {
  const [events, setEvents] = useState<VaultEvent[]>([]);

  useEffect(() => {
    const unlisten = listen<VaultEvent>("vault_event", (event) => {
      setEvents((prev) => [
        { ...event.payload, timestamp: new Date().toLocaleTimeString() },
        ...prev.slice(0, 49),
      ]);
    });
    return () => { unlisten.then((f) => f()); };
  }, []);

  return (
    <main className="container">
      <header>
        <h1>VAULT AUTHORITY <span className="version">v1.2</span></h1>
        <div className="status-indicator">SYSTEM ACTIVE</div>
      </header>
      <div className="dashboard-grid">
        <StatusPanel lastEvent={events[0]} />
        <EventFeed events={events} />
      </div>
    </main>
  );
}
