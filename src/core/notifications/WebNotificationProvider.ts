import type { NotificationProvider } from "./NotificationProvider";

export class WebNotificationProvider implements NotificationProvider {
  providerName(): string {
    return "web-notification";
  }

  async notify(title: string, message: string): Promise<void> {
    if (!("Notification" in window)) {
      alert(`${title}\n${message}`);
      return;
    }
    if (Notification.permission === "default") {
      await Notification.requestPermission();
    }
    if (Notification.permission === "granted") {
      new Notification(title, { body: message });
      return;
    }
    alert(`${title}\n${message}`);
  }
}
