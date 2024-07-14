import { setTimeout } from 'node:timers/promises'
import { HTTP_STATUS } from '../constants/http'
import {
  MESSAGE_200_OK,
  MESSAGE_400_BAD_REQUEST,
  MESSAGE_401_UNAUTHORIZED,
  MESSAGE_403_FORBIDDEN,
  MESSAGE_404_NOT_FOUND,
  MESSAGE_409_CONFLICT,
  MESSAGE_500_INTERNAL_SERVER_ERROR,
} from '../constants/messages'

export async function validatedBaseURL(url: string) {
  await setTimeout(500)
  if (url === undefined || url === null || url === '') {
    throw new Error('Base URL is required')
  }
  try {
    const response = await fetch(url)

    if (response.status === HTTP_STATUS.CONFLICT) {
      throw new Error(MESSAGE_409_CONFLICT)
    }

    if (response.status === HTTP_STATUS.UNAUTHORIZED) {
      throw new Error(MESSAGE_401_UNAUTHORIZED)
    }

    if (response.status === HTTP_STATUS.BAD_REQUEST) {
      throw new Error(MESSAGE_400_BAD_REQUEST)
    }

    if (response.status === HTTP_STATUS.FORBIDDEN) {
      throw new Error(MESSAGE_403_FORBIDDEN)
    }

    if (response.status === HTTP_STATUS.NOT_FOUND) {
      throw new Error(MESSAGE_404_NOT_FOUND)
    }

    if (response.status === HTTP_STATUS.INTERNAL_SERVER_ERROR) {
      throw new Error(MESSAGE_500_INTERNAL_SERVER_ERROR)
    }
  }
  catch (error) {
    throw new Error((error as Error).message)
  }

  return MESSAGE_200_OK
}
