# Change keys screenshots

![The lobby before this change, which a configured server also answered /setup with, light and dark](img/before-lobby.png)

![The lobby with the Change keys link at the end of the account row, light and dark](img/after-lobby.png)

![Change keys at /setup on a configured server, with the LiveKit endpoint and whether a Google key is set, light and dark](img/after-setup.png)

![The end of Change keys, where a save needs the code the server printed in its terminal, light and dark](img/after-code.png)

![Change keys after a save, with the restart notice every visit shows until the restart, light and dark](img/after-saved.png)

The screenshots come from Chromium at a 1280x860 window, light and dark from
the system scheme, on standalone loopback launches of 38d53a7 (before) and
2fed302 (after). LiveKit points at a stub at ws://localhost:7880 and every key
is made up; the lobby's random problem is seeded so the panels match.
First-time Setup renders byte-identical before and after in both schemes, so
it is not shown.
