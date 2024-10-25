import { appendFile } from "fs/promises";

export async function setSuiteEnv(key: string, value: string) {
    await appendFile("./.sink/env/.env.generated", `${key}=${value}\n`);
}