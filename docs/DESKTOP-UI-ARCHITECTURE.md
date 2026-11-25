# CU Desktop — Native Desktop UI Architecture

Status: proposed implementation architecture  
Scope: Windows desktop presentation only. The shared CU business logic, backend, data model, and mobile/web experiences remain authoritative and unchanged.

## 1. Product decision

CU Desktop is not a scaled-up phone layout and not a second CU application. It is a desktop presentation of the same CU product, with a separate information architecture optimized for keyboard, mouse, wide screens, multitasking, and long working sessions.

The desktop build marker already exists in the private CU source. Desktop-only UI must be selected through that marker. Do not infer desktop from generic WebView detection and do not broaden Capacitor runtime predicates.

## 2. Application frame

Use a four-region adaptive frame:

1. **App rail (64–76 px):** CU identity, primary module destinations, CU/Work mode switch, global create action, profile/settings.
2. **Navigation pane (240–320 px):** context-sensitive lists for the selected module: conversations, channels, folders, trips, saved items, etc. It can collapse to icons or hide when not useful.
3. **Main workspace (flexible):** the active destination and its desktop-specific content layout.
4. **Inspector pane (280–360 px, optional):** details, participants, trip information, files, properties, or contextual AI. Resizable and dismissible.

The inspector is not mandatory on every screen. The workspace must remain useful when the navigation pane or inspector is closed.

## 3. Modes

### CU mode

A desktop-native way to use familiar CU modules. It keeps the existing product capabilities and navigation concepts but presents them in layouts designed for desktop.

Examples:
- **Chats:** conversation list in the navigation pane; selected conversation in the workspace; contact/profile/shared media in the inspector.
- **Channels:** channel list; readable feed/article view; optional channel details and related content in the inspector.
- **CU News:** editorial list beside a full reading view; do not force users to open a browser for CU-owned articles.
- **Trips / Planner:** trip or itinerary list beside a calendar, map, schedule, or route canvas.
- **AI:** conversation and prompt history alongside the active result; optional selected context in the inspector.
- **Translator:** source and translated text side by side, with language and session controls visible.
- **Services / excursions:** discovery list beside details and booking actions.

Do not invent replacement business rules. Each module must call the existing CU domain logic, APIs, authorization, and backend.

### Work mode

A persistent multitasking workspace, not a second data system.

- Workspace tabs or named work areas.
- Split panes that can show a chat next to an article, route, trip, or document.
- Resizable pane boundaries and clear focus/active-pane states.
- Contextual AI can use only content the user is authorized to access and explicitly selects or attaches.
- Opening the same entity in multiple panes must reference the same underlying entity and permissions.
- Layout preferences may be stored locally per desktop user/device; business data remains in the existing CU backend.

## 4. Interaction model

- Mouse-first controls with full keyboard accessibility.
- Global search and command palette.
- Predictable shortcuts for navigation, search, new chat, pane focus, and close pane.
- Context menus on relevant rows and content.
- Drag/drop only where the destination and result are explicit.
- Resizable navigation and inspector panes with sensible minimum widths.
- Remember pane visibility, widths, selected workspace, and density locally.
- Do not make essential actions available only through hover.

Initial shortcut proposals (final mapping must avoid OS/browser conflicts):
- Ctrl/Cmd+K: command palette.
- Ctrl/Cmd+P: global search or quick switcher (choose one consistently).
- Ctrl/Cmd+1…9: switch primary destinations.
- Ctrl/Cmd+\\: toggle navigation pane.
- Ctrl/Cmd+Shift+\\: toggle inspector.
- Esc: close the topmost transient surface, then clear selection.

## 5. Responsive desktop breakpoints

These are desktop layout breakpoints, not mobile breakpoints:

- **Compact window (< 1100 px):** app rail + one content pane; navigation and inspector become overlays/drawers.
- **Standard (1100–1439 px):** app rail + navigation + workspace; inspector opens as a drawer or replaces navigation.
- **Wide (1440–1799 px):** app rail + navigation + workspace + optional inspector.
- **Ultra-wide (>= 1800 px):** same regions with a capped readable content width; do not stretch message lines or article text indefinitely.

The minimum supported window size must be revisited after the new layouts are implemented. Avoid fixing the minimum width based only on the old mobile-like screen.

## 6. Visual system

Create desktop-only design tokens and components, while reusing CU brand colors, typography, icons, and theme semantics where practical.

Token groups:
- app frame dimensions and pane widths;
- spacing, radii, borders, elevation, and focus rings;
- typography scale for dense data and long-form reading;
- surface/background levels;
- semantic states (success, warning, danger, unread, active);
- compact / comfortable density.

The desktop UI should feel like CU, not a clone of Slack, Arc, or Notion. Reference products are interaction inspiration only.

## 7. Technical boundary and integration

- Implement the actual desktop UI in the **private CU source repository**, because that repository owns the React application and its modules.
- Keep the public Desktop repository responsible for Tauri/Rust shell, Windows lifecycle, native integrations, packaging, and desktop-level documentation/tests.
- Load desktop-only components/styles behind the existing desktop build marker/runtime predicate.
- Do not fork module logic or create a second auth, chat, notification, booking, or user-data implementation.
- Keep desktop-specific preferences separate from authoritative business data.
- Do not change web SSR/PWA, Android, or iOS behavior. Add explicit target tests proving the desktop branch is isolated.

## 8. Implementation sequence

1. **Desktop frame:** app rail, navigation pane, workspace canvas, optional inspector, mode switch, theme/density tokens.
2. **First reference module — Chats:** three-pane layout, conversation list, desktop message composer, keyboard/focus behavior.
3. **Channels and CU News:** list/detail reading layouts and in-app full article reading.
4. **Planner / trips / services:** wide-screen calendar, itinerary, map and detail layouts.
5. **AI and Translator:** side-by-side work surfaces and explicit context selection.
6. **Work mode:** tabs/workspaces, split panes, persistence of local layout preferences.
7. **Windows validation:** high-DPI, resize, keyboard, notifications/deep links, tray lifecycle, media and file flows.

Each phase must be tested in a Windows build. Do not call the product desktop-native merely because it compiles in Tauri.

## 9. Acceptance criteria

- Desktop displays a distinct app frame and desktop navigation at normal PC window sizes.
- No primary desktop screen is merely a phone-width column centered in a large window.
- Chats, channels, articles, and trip planning use desktop-appropriate multi-pane layouts.
- Users can collapse/resize navigation and inspector regions.
- Work mode can place two supported CU entities side by side without duplicating their data.
- The same account, authorization, and backend data are used in CU and Work modes.
- Web, Android, and iOS build outputs and runtime behavior remain unchanged.
- Windows installer smoke tests cover navigation, authentication, core module rendering, window resize, and tray behavior.

## 10. Explicit non-goals

- Rewriting CU business logic or backend.
- Creating separate desktop user accounts, chat storage, or booking pipelines.
- Changing the mobile UI to make it fit desktop.
- Copying private CU application source into this public repository.
- Promising that every module is desktop-adapted in the first release.
