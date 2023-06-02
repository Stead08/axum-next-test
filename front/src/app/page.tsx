

export default function Home() {
  return (
      <main>
        <div className="flex-col">
          <h1 className="text-3xl font-bold">Next.js + TypeScript + Axum</h1>
          <a className="text-2xl underline" href={'/api/health'}>API Health Check</a>
        </div>
      </main>
  )
}
