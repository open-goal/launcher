import { redirect } from "@sveltejs/kit";

export const ssr = false; // client-only

export const load = () => {
  // Safe in client-only mode:
  const last = localStorage.getItem("lastGame") as
    | "jak1"
    | "jak2"
    | "jak3"
    | null;
  const target = last ?? "jak1";
  throw redirect(307, `/game/${target}`);
};
