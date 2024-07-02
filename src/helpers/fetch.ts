import { setTimeout } from 'node:timers/promises'

interface IConfig {
  method: string
  headers?: HeadersInit
  body?: unknown
  baseURL?: string
  endpoint?: string
}

export async function buildFetchToServer(_config: IConfig) {
  await setTimeout(500)
}
