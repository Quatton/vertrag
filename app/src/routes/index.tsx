import { createFileRoute } from '@tanstack/react-router'
import { css } from 'styled-system/css'

export const Route = createFileRoute('/')({
  component: App,
})

function App() {
  return (
    <main className={css({
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      height: '100vh',
      fontSize: 'xl',
      color: 'text',
      backgroundColor: 'background',
    })}>
      <h1 className={css({
        fontSize: '4xl',
        fontWeight: 'bold',
        color: 'primary',
      })}>
        Welcome to Panda CSS with React Router!
      </h1>
      <p className={css({
        fontSize: 'lg',
        color: 'text',
      })}>
        This is a simple example of using Panda CSS with React Router.
      </p>
    </main>
  )
}
