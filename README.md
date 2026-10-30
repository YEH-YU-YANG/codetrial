# Look-away countdown screenshots

![The last second of the task-mode look-away countdown before this change, reading "Face the screen: the attempt ends in 1 seconds."](img/before-countdown.png)

![The same moment after this change, reading "Face the screen: the attempt ends in 1 second."](img/after-countdown.png)

The screenshots come from Chromium at a 1280x720 window, cropped to the
countdown, on 1bb3deb (before) and 1cf9c2c (after). They are taken by the
look-away countdown test in tests/browser/task-workspace.test.js, 5.5 seconds
into a look-away against its 6-second threshold, with the test's stub task,
camera and face detector.
