import { isNativeMobileRuntime } from "../core/runtime/RuntimeProvider";
import { appFeatures } from "./projectConfig";

export type MenuSection = "dashboard" | "cadastro" | "sistema" | "ferramentas" | "documentacao";

export interface MenuItemConfig {
  title: string;
  route: string;
  permission?: string;
  feature?: keyof typeof appFeatures;
  section: MenuSection;
  eyebrow?: string;
  description?: string;
  desktopOnly?: boolean;
}

// Operational navigation is product-specific. Legacy administration routes remain intact.
export const menuItems: MenuItemConfig[] = [
  { title: "Visão geral", route: "/", permission: "dashboard:view", section: "dashboard" },
  { title: "Simulação", route: "/simulador-balancas", permission: "config:view", section: "ferramentas", description: "Configure protocolos, portas e instâncias de balanças TCP e RS232." },
  { title: "Playground", route: "/playground", permission: "config:manage", section: "ferramentas", description: "Inicie modelos salvos e controle cada balança." },
  { title: "Parâmetros", desktopOnly: true, route: "/sistema", permission: "config:view", feature: "systemSettings", section: "sistema" },
  { title: "Usuários", route: "/usuarios", permission: "usuarios:view", section: "sistema" },
  { title: "Perfis de acesso", route: "/perfis", permission: "perfis:view", section: "sistema" },
  { title: "Logs", route: "/logs", permission: "config:view", feature: "logs", section: "sistema" },
  { title: "Diagnósticos", desktopOnly: true, route: "/runtime", permission: "config:view", section: "sistema" },
  { title: "Banco de dados", route: "/sistema/banco", permission: "config:view", feature: "databaseSettings", section: "sistema" },
  { title: "Guia de simulação", route: "/documentacao/guia", feature: "userGuide", section: "documentacao" },
  { title: "Documentação da API", desktopOnly: true, route: "/documentacao/scalar", permission: "config:view", feature: "scalarDocs", section: "documentacao" },
  { title: "Ficha técnica", route: "/ficha-tecnica", permission: "config:view", feature: "technicalSheet", section: "documentacao" },
  { title: "Sobre", route: "/sobre", feature: "about", section: "documentacao" },
];

export function isFeatureEnabled(feature?: keyof typeof appFeatures): boolean {
  return feature ? appFeatures[feature] !== false : true;
}

export function visibleMenuItems(): MenuItemConfig[] {
  const nativeMobile = isNativeMobileRuntime();
  return menuItems.filter((item) => isFeatureEnabled(item.feature) && !(nativeMobile && item.desktopOnly));
}


export function findMenuItemByRoute(route: string): MenuItemConfig | undefined {
  const normalize = (value: string) => {
    const clean = (value || "/").replace(/\/+$/, "");
    return clean || "/";
  };

  const target = normalize(route);
  return visibleMenuItems().find((item) => normalize(item.route) === target);
}
