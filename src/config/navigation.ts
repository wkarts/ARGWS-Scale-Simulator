import { appFeatures } from "./projectConfig";

export type MenuSection = "dashboard" | "cadastro" | "simulador" | "sistema" | "ferramentas" | "documentacao";

export interface MenuItemConfig {
  title: string;
  route: string;
  permission?: string;
  feature?: keyof typeof appFeatures;
  section: MenuSection;
}

export const menuItems: MenuItemConfig[] = [
  { title: "Visão geral", route: "/", permission: "dashboard:view", section: "dashboard" },

  { title: "Empresas", route: "/empresas", permission: "empresas:view", section: "cadastro" },
  { title: "Simulador de balanças", route: "/simulador-balancas", permission: "config:view", section: "simulador" },

  { title: "Usuários", route: "/usuarios", permission: "usuarios:view", section: "cadastro" },
  { title: "Perfis de usuários", route: "/perfis", permission: "perfis:view", section: "cadastro" },
  { title: "Logs", route: "/logs", permission: "config:view", feature: "logs", section: "sistema" },
  { title: "Parâmetros", route: "/sistema", permission: "config:view", feature: "systemSettings", section: "sistema" },
  { title: "Banco de dados", route: "/sistema/banco", permission: "config:view", feature: "databaseSettings", section: "sistema" },
  { title: "Diagnósticos", route: "/runtime", permission: "config:view", section: "sistema" },


  { title: "Sobre", route: "/sobre", feature: "about", section: "documentacao" },
  { title: "Guia do usuário", route: "/documentacao/guia", feature: "userGuide", section: "documentacao" },
];

export function isFeatureEnabled(feature?: keyof typeof appFeatures): boolean {
  return feature ? appFeatures[feature] !== false : true;
}

export function visibleMenuItems(): MenuItemConfig[] {
  return menuItems.filter((item) => isFeatureEnabled(item.feature));
}
