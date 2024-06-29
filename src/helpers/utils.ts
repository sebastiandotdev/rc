import process from 'node:process'
import { spinner as _spinner, cancel, isCancel, select } from '@clack/prompts'
import chalk from 'chalk'

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
