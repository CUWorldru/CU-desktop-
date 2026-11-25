import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";
import "./style.css";

const app = document.querySelector<HTMLDivElement>("#app");

if (!app) throw new Error("CU Desktop root element was not found");

app.innerHTML = `
  <main class="shell">
    <div class="brand-mark" aria-hidden="true">CU</div>
    <p class="eyebrow">321 CENTURY · SEEYOUMORE</p>
    <h1>CU Desktop</h1>
    <p class="lead">Windows desktop shell is running.</p>
    <section class="status" aria-live="polite">
      <span class="status-dot"></span>
      <div>
        <strong>Desktop foundation</strong>
        <p>Local Tauri frontend loaded. The private CU application bundle is not connected yet.</p>
      </div>
    </section>
    <button id="notification-test" type="button">Test Windows notification</button>
    <p id="feedback" class="feedback" role="status"></p>
  </main>
`;

const button = document.querySelector<HTMLButtonElement>("#notification-test");
const feedback = document.querySelector<HTMLParagraphElement>("#feedback");

button?.addEventListener("click", async () => {
  if (!feedback || !button) return;
  button.disabled = true;
  feedback.textContent = "Checking notification permission…";
  try {
    let granted = await isPermissionGranted();
    if (!granted) granted = (await requestPermission()) === "granted";
    if (!granted) {
      feedback.textContent = "Windows notification permission was not granted.";
      return;
    }
    sendNotification({
      title: "CU Desktop",
      body: "Windows notification bridge is responding.",
    });
    feedback.textContent = "Notification request sent to Windows.";
  } catch (error) {
    feedback.textContent = `Notification test failed: ${String(error)}`;
  } finally {
    button.disabled = false;
  }
});
