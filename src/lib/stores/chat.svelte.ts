import type { Message } from '$lib/types';

const messages = $state<Message[]>([
  {
    id: '1',
    content: '# Welcome!\n\nHello! How can I help you today? I can assist with:\n\n- Programming questions\n- Code reviews\n- Debugging issues\n- Best practices',
    role: 'assistant',
    timestamp: new Date('2024-01-01T00:00:00Z')
  },
  {
    id: '2',
    content: 'I have a question about programming.',
    role: 'user',
    timestamp: new Date('2024-01-01T00:00:05Z')
  },
  {
    id: '3',
    content: 'Of course! I\'d be happy to help. Here\'s an example of how I can format code:\n\n```javascript\n// Example function\nfunction greet(name) {\n  return `Hello ${name}!`;\n}\n```\n\nI can also use **bold**, *italic*, and `inline code`. What would you like to know?',
    role: 'assistant',
    timestamp: new Date('2024-01-01T00:00:10Z')
  }
],)

export const chatStore = {
  addMessage(message: Message) {
    messages.push(message)
  },
  get messages() {
    return messages
  }
}
