/** @type {import('@sveltejs/kit').Handle} */
// disable server side rendering
export async function handle({ event, resolve }) {
  return resolve(event, { ssr: false });
}