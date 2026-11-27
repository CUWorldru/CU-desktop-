import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";
import "./style.css";

type CompanionAccount = {
  user: {
    id: string;
    profile: {
      display_name?: string | null;
      full_name?: string | null;
      first_name?: string | null;
      username?: string | null;
      avatar_url?: string | null;
    } | null;
  };
};

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) throw new Error("CU Desktop root element was not found");

app.innerHTML = `
  <main class="shell">
    <div class="brand-mark" aria-hidden="true">CU</div>
    <p class="eyebrow">321 CENTURY · SEEYOUMORE</p>
    <h1>CU Desktop</h1>
    <p class="lead">Windows desktop shell</p>
    <section class="status" aria-live="polite">
      <span class="status-dot"></span>
      <div>
        <strong id="account-title">Companion session</strong>
        <p id="account-status">Checking this computer's CU connection…</p>
      </div>
    </section>
    <div class="actions">
      <button id="account-refresh" type="button">Check CU account</button>
      <button id="notification-test" class="secondary" type="button">Test notification</button>
    </div>
    <p id="feedback" class="feedback" role="status"></p>
  </main>
`;

const feedback = document.querySelector<HTMLParagraphElement>("#feedback");
const accountTitle = document.querySelector<HTMLElement>("#account-title");
const accountStatus = document.querySelector<HTMLElement>("#account-status");
const accountButton = document.querySelector<HTMLButtonElement>("#account-refresh");
const notificationButton = document.querySelector<HTMLButtonElement>("#notification-test");

async function loadCompanionAccount() {
  if (!feedback || !accountTitle || !accountStatus || !accountButton) return;
  accountButton.disabled = true;
  feedback.textContent = "Verifying the Windows Companion session…";
  try {
    const present = await invoke<boolean>("companion_session_is_present");
    if (!present) {
      accountTitle.textContent = "Computer not connected";
      accountStatus.textContent = "Scan the pairing QR code from CU on your phone to authorize this computer.";
      feedback.textContent = "No Companion session is stored on this computer.";
      return;
    }
    let account: CompanionAccount;
    try {
      account = await invoke<CompanionAccount>("companion_account_me");
    } catch (error) {
      const message = String(error);
      if (message.includes("COMPANION_UNAUTHORIZED")) {
        await invoke("companion_session_refresh");
        account = await invoke<CompanionAccount>("companion_account_me");
      } else {
        throw error;
      }
    }
    const profile = account.user.profile;
    const label = profile?.display_name || profile?.full_name || profile?.first_name || profile?.username || "CU account";
    accountTitle.textContent = "CU account connected";
    accountStatus.textContent = label;
    feedback.textContent = "Windows session verified by the Companion server.";
  } catch (error) {
    accountTitle.textContent = "Connection check failed";
    accountStatus.textContent = "The saved session could not be verified.";
    feedback.textContent = String(error);
  } finally {
    accountButton.disabled = false;
  }
}

accountButton?.addEventListener("click", () => void loadCompanionAccount());
void loadCompanionAccount();

notificationButton?.addEventListener("click", async () => {
  if (!feedback || !notificationButton) return;
  notificationButton.disabled = true;
  feedback.textContent = "Checking notification permission…";
  try {
    let granted = await isPermissionGranted();
    if (!granted) granted = (await requestPermission()) === "granted";
    if (!granted) {
      feedback.textContent = "Windows notification permission was not granted.";
      return;
    }
    sendNotification({ title: "CU Desktop", body: "Windows notification bridge is responding." });
    feedback.textContent = "Notification request sent to Windows.";
  } catch (error) {
    feedback.textContent = `Notification test failed: ${String(error)}`;
  } finally {
    notificationButton.disabled = false;
  }
});
