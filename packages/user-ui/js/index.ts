import { initializeApp } from "firebase/app";
import {
  GoogleAuthProvider,
  getAuth,
  signInWithPopup,
  signOut,
} from "firebase/auth";

import {
  ConsoleLogger,
  LogLevel,
  DefaultDeviceController,
  DefaultMeetingSession,
  MeetingSessionConfiguration,
} from "amazon-chime-sdk-js";

const firebase = {
  initializeApp,
  getAuth,
  signInWithPopup,
  signOut,
  GoogleAuthProvider,
};

declare global {
  interface Window {
    firebase: any;
    chime: any;
  }
}

const chime = {
  ConsoleLogger,
  LogLevel,
  DefaultDeviceController,
  DefaultMeetingSession,
  MeetingSessionConfiguration,
};

window.firebase = firebase;

window.chime = chime;
