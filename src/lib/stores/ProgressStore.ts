import { writable } from "svelte/store";

export type ProgressStatus =
  | "inactive"
  | "success"
  | "queued"
  | "pending"
  | "failed";

export interface ProgressStep {
  status: ProgressStatus;
  label: String;
}

interface ProgressTracker {
  currentStep: number;
  overallStatus: ProgressStatus;
  steps: ProgressStep[];
  logs: string[];
  error?: string | undefined;
}

const storeValue: ProgressTracker = {
  currentStep: 0,
  overallStatus: "inactive",
  steps: [],
  logs: [],
};

function createProgressTracker() {
  const { subscribe, set, update } = writable<ProgressTracker>(storeValue);

  return {
    subscribe,
    init: (steps: ProgressStep[]) =>
      update((val) => {
        val.currentStep = 0;
        val.overallStatus = "inactive";
        val.steps = steps;
        val.logs = [];
        return val;
      }),
    start: () =>
      update((val) => {
        // Mark the current step (assumed to be the first) as pending
        val.overallStatus = "pending";
        val.steps[val.currentStep].status = "pending";
        return val;
      }),
    proceed: () =>
      update((val) => {
        // Mark the current step as completed successfully, move onto the next
        val.steps[val.currentStep].status = "success";
        val.currentStep++;
        // Check if we're done or not
        if (val.currentStep >= val.steps.length) {
          val.overallStatus = "success";
        } else {
          val.steps[val.currentStep].status = "pending";
        }
        return val;
      }),
    halt: () =>
      update((val) => {
        // Fail the current step and the overall job
        val.overallStatus = "failed";
        val.steps[val.currentStep].status = "failed";
        return val;
      }),
    appendLogs: (logs: string[]) =>
      update((val) => {
        val.logs = [...val.logs, ...logs];
        return val;
      }),
    clear: () => {
      set({
        currentStep: 0,
        overallStatus: "inactive",
        steps: [],
        logs: [],
      });
    },
    fail: (err: string | undefined) =>
      update((v) => {
        v.overallStatus = "failed";
        v.steps[v.currentStep].status = "failed";
        v.error = err;
        return v;
      }),
  };
}

export const progressTracker = createProgressTracker();
