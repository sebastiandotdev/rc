import { setTimeout } from 'node:timers/promises'
import * as p from '@clack/prompts'
import chalk from 'chalk'
import { selectCommand, spinner } from '../helpers/utils'

const httpMethods = [
  {
    hint: 'Use method GET to fetch data from the server',
    label: chalk.gray('GET'),
    value: 'GET',
  },
  {
    hint: 'Use method POST to send data to the server',
    label: chalk.gray('POST'),
    value: 'POST',
  },
  {
    hint: 'Use method PUT to update data on the server',
    label: chalk.gray('PUT'),
    value: 'PUT',
  },
  {
    hint: 'Use method DELETE to remove data from the server',
    label: chalk.gray('DELETE'),
    value: 'DELETE',
  },
]

export async function initAction(baseURL: string | undefined) {
  p.intro(chalk.bold.blue('⚡ Initialize a new REST Client powered by RC ⚡'))
  p.log.step(chalk.bold.green('Building a REST Client is as easy as 1-2-3! 🚀'))
  let isNotURL = false

  await selectCommand({
    initialValue: 'GET',
    message: 'Select the HTTP method to use',
    options: httpMethods,
  })

  if (!baseURL) {
    spinner.start('checking if you have a base URL...')

    const isCoompleted = await setTimeout(3000, true)

    if (isCoompleted) {
      isNotURL = true
    }
  }

  if (isNotURL) {
    spinner.stop(chalk.bold.red('Failed. Please provide a base URL to continue.'))
  }
}
