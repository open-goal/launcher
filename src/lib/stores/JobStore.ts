import { get, writable } from "svelte/store";

export type StepStatus =
  | "inactive"
  | "success"
  | "queued"
  | "pending"
  | "failed";

export type OverallStatus = "inactive" | "pending" | "success" | "failed";

export interface JobStep {
  status: StepStatus;
  label: string;
  task: () => Promise<boolean>;
}

interface JobTracker {
  currentStep: number;
  overallStatus: OverallStatus;
  steps: JobStep[];
  logs: string[];
  failureReason: string | undefined;
}

const storeValue: JobTracker = {
  currentStep: 0,
  overallStatus: "inactive",
  steps: [],
  logs: [],
  failureReason: undefined
};

function createJobTracker() {
  const { subscribe, set, update } = writable<JobTracker>(storeValue);

  return {
    subscribe,
    init: (steps: JobStep[]) =>
      update((val) => {
        val.currentStep = 0;
        val.overallStatus = "inactive";
        val.steps = steps;
        val.logs = [];
        val.failureReason = undefined;
        return val;
      }),
    clear: () => {
      set({
        currentStep: 0,
        overallStatus: "inactive",
        steps: [],
        logs: [],
        failureReason: undefined
      });
    },
    start: async () => {
      let storeState = get({ subscribe });
      update((val) => {
        val.overallStatus = "pending";
        return val;
      });
      // TODO - change to a while?
      for (const step of storeState.steps) {
        update((val) => {
          val.steps[val.currentStep].status = "pending"
          return val;
        });
        const result = await step.task();
        if (!result) {
          update((val) => {
            val.overallStatus = "failed";
            val.steps[val.currentStep].status = "failed"
            return val;
          });
        } else {
          update((val) => {
            val.steps[val.currentStep].status = "success"
            val.currentStep++;
            // Check if we're done or not
            if (val.currentStep >= val.steps.length) {
              val.overallStatus = "success";
            } else {
              val.steps[val.currentStep].status = "pending";
            }
            return val;
          });
        }
        // incase something has changed (there are no steps)
        storeState = get({ subscribe });
      }
    },
    updateFailureReason: (reason: string | undefined | null) =>
      update((val) => {
        if (reason === null) {
          val.failureReason = undefined;
        } else {
          val.failureReason = reason;
        }
        return val;
      }),
    appendLogs: (logs: string[]) =>
      update((val) => {
        val.logs = [...val.logs, ...logs];
        return val;
      }),
  };
}

export const jobTracker = createJobTracker();
