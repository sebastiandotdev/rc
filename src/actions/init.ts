import { setTimeout } from 'node:timers/promises'
import * as p from '@clack/prompts'
import chalk from 'chalk'
import { selectCommand, taskCommand, textCommand } from '../helpers/utils'

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

async function isURLValid(URL: string | symbol) {
  await setTimeout(500)

  if (!URL || URL === '') {
    throw new Error('Base URL not found')
  }

  if (typeof URL !== 'string') {
    throw new TypeError('Base URL must be a string')
  }

  if (!URL.startsWith('http://') || !URL.startsWith('https://')) {
    throw new Error('Invalid URL')
  }
}

export async function initAction() {
  p.intro(chalk.bold.blue('⚡ Initialize a new REST Client powered by RC ⚡'))
  p.log.step(chalk.bold.green('Building a REST Client is as easy as 1-2-3! 🚀'))

  const baseURL = await textCommand({
    message: 'Enter the base URL of the API you want to interact with',
    placeholder: 'https://api.example.com',
  })

  await selectCommand({
    initialValue: 'GET',
    message: 'Select the HTTP method to use',
    options: httpMethods,
  })

  await taskCommand({
    task: isURLValid(baseURL),
    text: 'Checking if you have a base URL...',
    failText: 'Failed. Please provide a base URL to continue.',
    successText: 'Base URL found!',
  })

  p.log.info('You have successfully initialized a new REST Client! 🎉')
}
