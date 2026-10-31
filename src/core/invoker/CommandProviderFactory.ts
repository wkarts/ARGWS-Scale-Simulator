import { isTauriRuntime } from "../runtime/RuntimeProvider";
import type { CommandArgs, CommandProvider } from "./CommandProvider";
import { TauriCommandProvider } from "./TauriCommandProvider";
import { WebCommandProvider } from "./WebCommandProvider";

let provider: CommandProvider | null = null;

export function useCommandProvider(): CommandProvider {
  if (!provider) {
    provider = isTauriRuntime() ? new TauriCommandProvider() : new WebCommandProvider();
  }
  return provider;
}

export async function invokeAppCommand<T>(command: string, args?: CommandArgs): Promise<T> {
  return useCommandProvider().invoke<T>(command, args);
}
