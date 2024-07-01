import process from 'node:process'
import { spinner as _spinner, cancel, isCancel, select, text } from '@clack/prompts'
import chalk from 'chalk'

export interface TaskClackOptions<T> {
  text: string
  task: PromiseLike<T>
  successText?: string
  failText?: string
}

export const spinner = _spinner()

export function cancelCommand(value: any) {
  if (isCancel(value)) {
    cancel(`${chalk.red('✖')} Operation cancelled`)
    process.exit(0)
  }
}

export const selectCommand: typeof select = async (opts) => {
  const result = await select(opts)

  cancelCommand(result)

  return result
}

export const textCommand: typeof text = async (opts) => {
  const result = (await text(opts)) as string

  cancelCommand(result)

  return result
}

export async function taskCommand<T>(opts: TaskClackOptions<T>) {
  const { failText, successText, task, text } = opts

  let result: string | null = null

  try {
    spinner.start(text)
    result = await (task as any)
    spinner.stop(successText)
  }
  catch (error) {
    spinner.stop()
    cancel(failText)
  }

  return result
}
