/**
 * Centralized Tauri IPC Service Layer.
 * Provides strongly-typed asynchronous client methods to invoke Rust backend capabilities.
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  LauncherInfo,
  Account,
  Instance,
  MinecraftVersionSummary,
  ModItem,
  LauncherSettings,
  JavaInstallation,
  GpuDevice,
  PlayerSkinPayload,
  UpdateCheckResult,
  ServerPingResult,
  FavoriteServer,
} from "../types";

export const TauriService = {
  // --- Launcher Info ---
  getLauncherInfo(): Promise<LauncherInfo> {
    return invoke("get_launcher_info");
  },

  // --- Accounts ---
  getAccounts(): Promise<Account[]> {
    return invoke("get_accounts");
  },

  selectActiveAccount(accountId: string): Promise<void> {
    return invoke("select_active_account", { accountId });
  },

  addOfflineAccount(username: string): Promise<Account> {
    return invoke("add_offline_account", { username });
  },

  logoutAccount(accountId: string): Promise<void> {
    return invoke("logout_account", { accountId });
  },

  startMicrosoftLogin(): Promise<{ auth_url: string; flow_id: string }> {
    return invoke("start_microsoft_login");
  },

  completeMicrosoftLogin(flowId: string, authCode: string): Promise<Account> {
    return invoke("complete_microsoft_login", { flowId, authCode });
  },

  // --- Instances & Versions ---
  getInstances(): Promise<Instance[]> {
    return invoke("get_instances");
  },

  selectActiveInstance(instanceId: string): Promise<void> {
    return invoke("select_active_instance", { instanceId });
  },

  createInstance(name: string, mcVersion: string, modLoader: string): Promise<Instance> {
    return invoke("create_instance", { name, mcVersion, modLoader });
  },

  deleteInstance(instanceId: string): Promise<void> {
    return invoke("delete_instance", { instanceId });
  },

  updateInstanceMemory(id: string, maxMemoryMb: number): Promise<void> {
    return invoke("update_instance_memory", { id, maxMemoryMb });
  },

  getAllMinecraftVersions(): Promise<MinecraftVersionSummary[]> {
    return invoke("get_all_minecraft_versions");
  },

  // --- Mod Management ---
  getInstalledMods(instanceId: string): Promise<ModItem[]> {
    return invoke("get_installed_mods", { instanceId });
  },

  toggleMod(instanceId: string, filename: string, enabled: boolean): Promise<void> {
    return invoke("toggle_mod", { instanceId, filename, enabled });
  },

  deleteMod(instanceId: string, filename: string): Promise<void> {
    return invoke("delete_mod", { instanceId, filename });
  },

  bulkToggleMods(instanceId: string, enableAll: boolean): Promise<number> {
    return invoke("bulk_toggle_mods", { instanceId, enableAll });
  },

  importLocalMod(instanceId: string): Promise<string | null> {
    return invoke("import_local_mod", { instanceId });
  },

  searchModrinth(params: {
    query: string | null;
    loader: string | null;
    gameVersion?: string | null;
    category?: string | null;
    sortBy?: string | null;
    limit?: number | null;
    projectType?: string | null;
  }): Promise<any[]> {
    return invoke("search_modrinth", params);
  },

  installModrinthMod(
    instanceId: string,
    projectId: string,
    slug?: string | null,
    title?: string | null,
    contentType?: string | null
  ): Promise<string> {
    return invoke("install_modrinth_mod", { instanceId, projectId, slug: slug || null, title: title || null, contentType: contentType || null });
  },

  uninstallModrinthMod(instanceId: string, slug: string): Promise<void> {
    return invoke("uninstall_modrinth_mod", { instanceId, slug });
  },

  searchCurseforge(params: {
    query: string | null;
    loader: string | null;
    gameVersion?: string | null;
    categoryId?: number | null;
    sortBy?: string | null;
    limit?: number | null;
    classId?: number | null;
  }): Promise<any[]> {
    return invoke("search_curseforge", params);
  },

  installCurseforgeMod(
    instanceId: string,
    modId: number,
    slug?: string | null,
    title?: string | null,
    contentType?: string | null
  ): Promise<string> {
    return invoke("install_curseforge_mod", { instanceId, modId, slug: slug || null, title: title || null, contentType: contentType || null });
  },

  openModsFolder(instanceId?: string | null): Promise<void> {
    return invoke("open_mods_folder", { instanceId: instanceId || null });
  },

  getInstalledContent(instanceId: string, contentType: string): Promise<any[]> {
    return invoke("get_installed_content", { instanceId, contentType });
  },

  deleteInstalledContent(instanceId: string, contentType: string, filename: string): Promise<void> {
    return invoke("delete_installed_content", { instanceId, contentType, filename });
  },

  openContentFolder(instanceId: string, contentType: string): Promise<void> {
    return invoke("open_content_folder", { instanceId, contentType });
  },

  // --- Skins & Capes ---
  getPlayerSkin(username?: string | null, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("get_player_skin", { username: username || null, uuid: uuid || null });
  },

  pickSkinFile(): Promise<string | null> {
    return invoke("pick_skin_file");
  },

  pickCapeFile(): Promise<string | null> {
    return invoke("pick_cape_file");
  },

  applySkin(username: string, filePath: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("apply_skin", { username, uuid: uuid || null, filePath });
  },

  applySkinBase64(username: string, base64Data: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("apply_skin_base64", { username, uuid: uuid || null, base64Data });
  },

  applyCape(username: string, filePath: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("apply_cape", { username, uuid: uuid || null, filePath });
  },

  applyCapeBase64(username: string, base64Data: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("apply_cape_base64", { username, uuid: uuid || null, base64Data });
  },

  resetSkin(username: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("reset_skin", { username, uuid: uuid || null });
  },

  resetCape(username: string, uuid?: string | null): Promise<PlayerSkinPayload> {
    return invoke("reset_cape", { username, uuid: uuid || null });
  },

  forceSyncSkinToInstances(username: string, uuid?: string | null): Promise<string> {
    return invoke("force_sync_skin_to_instances", { username, uuid: uuid || null });
  },

  // --- Game Lifecycle ---
  launchGame(instanceId: string, directConnectServer?: string): Promise<void> {
    return invoke("launch_game", { instanceId, directConnectServer: directConnectServer || null });
  },

  stopGame(): Promise<void> {
    return invoke("stop_game");
  },

  getGameStatus(): Promise<string> {
    return invoke("get_game_status");
  },

  getLatestLogs(instanceId?: string | null): Promise<string> {
    return invoke("get_latest_logs", { instanceId: instanceId || null });
  },

  openLogsFolder(instanceId?: string | null): Promise<void> {
    return invoke("open_logs_folder", { instanceId: instanceId || null });
  },

  exportLogs(content: string): Promise<string | null> {
    return invoke("export_logs", { content });
  },

  reportCrash(params: {
    logs: string;
    crashSnippet?: string | null;
    instanceId?: string | null;
  }): Promise<boolean> {
    return invoke("report_crash", {
      logs: params.logs,
      crashSnippet: params.crashSnippet ?? null,
      instanceId: params.instanceId ?? null,
    });
  },

  // --- Hardware, Java & Settings ---
  detectJava(): Promise<JavaInstallation[]> {
    return invoke("detect_java");
  },

  installJava(majorVersion: number): Promise<string> {
    return invoke("install_java", { majorVersion });
  },

  pickCustomJava(): Promise<JavaInstallation | null> {
    return invoke("pick_custom_java");
  },

  detectGpus(): Promise<GpuDevice[]> {
    return invoke("detect_gpus");
  },

  getSettings(): Promise<LauncherSettings> {
    return invoke("get_settings");
  },

  saveSettings(settings: LauncherSettings): Promise<void> {
    return invoke("save_settings", { settings });
  },

  pickGameDirectory(): Promise<string | null> {
    return invoke("pick_game_directory");
  },

  resetGameDirectory(): Promise<LauncherSettings> {
    return invoke("reset_game_directory");
  },

  pickInstancesDirectory(): Promise<string | null> {
    return invoke("pick_instances_directory");
  },

  resetInstancesDirectory(): Promise<LauncherSettings> {
    return invoke("reset_instances_directory");
  },

  openFolder(path: string): Promise<void> {
    return invoke("open_folder", { path });
  },

  checkForUpdates(): Promise<UpdateCheckResult> {
    return invoke("check_for_updates");
  },

  openUrl(url: string): Promise<void> {
    return invoke("open_url", { url });
  },

  // --- Minecraft Servers & SLP Ping ---
  pingMinecraftServer(host: string, port?: number, timeoutMs?: number): Promise<ServerPingResult> {
    return invoke("ping_minecraft_server", { host, port: port || null, timeoutMs: timeoutMs || null });
  },

  getFavoriteServers(): Promise<FavoriteServer[]> {
    return invoke("get_favorite_servers");
  },

  saveFavoriteServer(name: string, host: string, port?: number, id?: string): Promise<FavoriteServer> {
    return invoke("save_favorite_server", { id: id || null, name, host, port: port || null });
  },

  deleteFavoriteServer(id: string): Promise<void> {
    return invoke("delete_favorite_server", { id });
  },

  // --- World Saves Manager ---
  getInstanceWorlds(instanceId: string): Promise<any[]> {
    return invoke("get_instance_worlds", { instanceId });
  },

  backupWorld(instanceId: string, worldFolder: string): Promise<string> {
    return invoke("backup_world", { instanceId, worldFolder });
  },

  deleteWorld(instanceId: string, worldFolder: string): Promise<void> {
    return invoke("delete_world", { instanceId, worldFolder });
  },

  // --- Screenshot Gallery ---
  getInstanceScreenshots(instanceId: string): Promise<any[]> {
    return invoke("get_instance_screenshots", { instanceId });
  },

  deleteScreenshot(instanceId: string, filename: string): Promise<void> {
    return invoke("delete_screenshot", { instanceId, filename });
  },

  // --- In-App Auto-Updater ---
  downloadAndInstallUpdate(downloadUrl: string): Promise<string> {
    return invoke("download_and_install_update", { downloadUrl });
  },
};
