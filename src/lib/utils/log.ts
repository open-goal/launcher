import {
  writeFile,
  createDir,
  readTextFile,
  readDir,
} from "@tauri-apps/api/fs";
import { logDir, join, dirname, basename } from "@tauri-apps/api/path";
import { fileExists } from "./file";

enum LogLevel {
  Debug = "debug",
  Info = "info",
  Warn = "warn",
  Error = "error",
}

export class Logger {
  metadata: Object = {};
  level: LogLevel = LogLevel.Debug;
  fileNamePrefix: string;

  logFileName: string = undefined;
  maxFileRotate: number = 10;
  buffer: string[] = [];
  lastFlush: Date = undefined;
  flushMillisecondInterval: number = 2500;

  constructor(metadata: Object, level: LogLevel, fileNamePrefix: string) {
    this.metadata = metadata;
    this.level = level;
    this.fileNamePrefix = fileNamePrefix;
  }

  private logMessage(msg: string, level: LogLevel, meta?: Object): string {
    let val = this.metadata;
    // Apply provided metadata
    if (meta !== undefined) {
      for (const [key, value] of Object.entries(meta)) {
        val[key] = `${value}`;
      }
    }
    val["level"] = level;
    val["message"] = msg;
    val["timestamp"] = new Date().toISOString();
    return JSON.stringify(val);
  }

  private async flushToFile(logData: string) {
    this.buffer.push(logData);
    // Flush if we havn't in X amount of time
    if (
      this.buffer.length > 0 &&
      (this.lastFlush === undefined ||
        this.lastFlush.valueOf() >
          new Date().valueOf() + this.flushMillisecondInterval)
    ) {
      await this.writeToFile();
      this.lastFlush = new Date();
      this.buffer = [];
    }
  }

  private async writeToFile() {
    // If we havn't figured out our log file name yet, figure it out now
    // This is so we don't have to clear our logs manually, just rotate files
    if (this.logFileName === undefined) {
      this.logFileName = await this.rotateLogFile();
    }

    // Check if the file exists and read it's contents if so
    const dir = await logDir();
    const fullPath = await join(dir, this.logFileName);
    const logExists = await fileExists(fullPath);
    let contents = "";
    if (logExists) {
      contents = await readTextFile(fullPath);
      if (contents === null || contents === undefined) {
        contents = "";
      }
    } else {
      await createDir(await dirname(fullPath), { recursive: true });
    }

    // Build up the string to append and write it
    this.buffer.forEach((data) => {
      if (data !== undefined && data !== null) {
        contents += data + "\n";
      }
    });
    await writeFile({ contents: contents, path: fullPath });
  }

  private async rotateLogFile(): Promise<string> {
    const dir = await logDir();
    const logFiles = await readDir(dir);
    let numLogs = 0;
    let oldestLogIndex = 0;
    for (let i = 0; i < logFiles.length; i++) {
      const logFile = logFiles[i];
      const logFileName = await basename(logFile.path);
      // prefix_number.log
      const [prefix, number] = logFileName.split("_");
      if (prefix === this.fileNamePrefix) {
        numLogs++;
        oldestLogIndex = Math.min(oldestLogIndex, parseInt(number));
      }
    }
    if (numLogs > this.maxFileRotate) {
      return `${this.fileNamePrefix}_${oldestLogIndex}.log`;
    } else {
      return `${this.fileNamePrefix}_${numLogs}.log`;
    }
  }

  child(meta: Object): Logger {
    let newLogger = new Logger(this.metadata, this.level, this.fileNamePrefix);
    let newMeta = this.metadata;
    for (const [key, value] of Object.entries(meta)) {
      newMeta[key] = `${value}`;
    }
    newLogger.metadata = newMeta;
    newLogger.logFileName = this.logFileName;
    return newLogger;
  }

  debug(msg: string, meta?: Object) {
    if (this.level <= LogLevel.Debug) {
      const logData = this.logMessage(msg, LogLevel.Debug, meta);
      console.log(logData);
      this.flushToFile(logData);
    }
  }

  info(msg: string, meta?: Object) {
    if (this.level <= LogLevel.Info) {
      const logData = this.logMessage(msg, LogLevel.Info, meta);
      console.log(logData);
      this.flushToFile(logData);
    }
  }

  warn(msg: string, meta?: Object) {
    if (this.level <= LogLevel.Warn) {
      const logData = this.logMessage(msg, LogLevel.Warn, meta);
      console.warn(logData);
      this.flushToFile(logData);
    }
  }

  error(msg: string, meta?: Object) {
    if (this.level <= LogLevel.Error) {
      const logData = this.logMessage(msg, LogLevel.Error, meta);
      console.error(logData);
      this.flushToFile(logData);
    }
  }
}

export const log = new Logger(
  {
    name: "launcher",
  },
  LogLevel.Debug,
  "launcher"
);

export const installLog = new Logger(
  {
    name: "launcher-install",
  },
  LogLevel.Debug,
  "launcher-install"
);
