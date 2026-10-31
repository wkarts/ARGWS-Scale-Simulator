import { projectConfig } from "./projectConfig";

function slugify(value: string): string {
  return value
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/[^a-zA-Z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "")
    .toLowerCase() || "app";
}

export const appBranding = {
  appName: projectConfig.app.name,
  shortName: projectConfig.app.shortName,
  productName: projectConfig.app.productName || projectConfig.app.name,
  windowTitle: projectConfig.app.windowTitle || projectConfig.app.name,
  subtitle: projectConfig.app.subtitle,
  description: projectConfig.app.description,
  developer: projectConfig.app.developer,
  identifier: projectConfig.app.identifier,
  localDataDir: projectConfig.app.localDataDir || slugify(projectConfig.app.identifier || projectConfig.app.name),
  storagePrefix: projectConfig.app.storagePrefix || slugify(projectConfig.app.identifier || projectConfig.app.name).replace(/_/g, "-"),
};

export function storageKey(key: string): string {
  return `${appBranding.storagePrefix}:${key}`;
}

export function appDbName(suffix = "desktop_web"): string {
  return `${appBranding.localDataDir}_${suffix}`.replace(/[^a-zA-Z0-9_]/g, "_");
}
