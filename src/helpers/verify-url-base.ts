import { setTimeout } from 'node:timers/promises'

export async function validatedBaseURL(url: string) {
  await setTimeout(500)
  if (url === undefined || url === null || url === '') {
    throw new Error('Base URL is required')
  }

  const response = await fetch(url)
  if (!response.ok) {
    throw new Error('Base URL is invalid')
  }

  if (response.status === 404) {
    throw new Error('Base URL is invalid')
  }

  return true
}
