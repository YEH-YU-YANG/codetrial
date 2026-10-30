# Cube Reset and Show captures

![Reset pressed while the cube is still turning, on main. Step 1: Apply R U R' U', and the cube starts turning. Step 2: Reset is pressed while it is still turning, and the cube shows solved. Step 3, two seconds later: the cube has turned again and reads "Moves so far: R U R' U'"](img/reset-issue.png)

![Reset pressed while the cube is still turning, before and after this change. Step 3, two seconds later: the cube before the change has turned again and reads "Moves so far: R U R' U'", while the cube after it stays solved](img/reset-storyboard.png)

![The same Reset at a third of real speed](img/reset-slow.png)

![Show 21653471111312 pressed while the cube is still turning. Two seconds later the cube before this change ends on 12673452213311 with "Moves so far: R U R' U'", while the cube after it stays on 21653471111312](img/show-storyboard.png)

The captures come from Chromium 156 at a 1280x1000 window on 3bbbcf5 (main,
before) and c4797e5 (this change, after), in the task-workspace browser test
harness with its stub task agent, camera and face detector and the cube
explainer turned on. Each run types R U R' U', presses Apply, and clicks Reset
or Show with the mouse about 0.1 s later, while the first turn is still
swinging, then takes a screenshot every 50 to 100 ms for 2.3 s. Each cell is
cut from those screenshots: the cube, the algorithm row and the history line,
and for Show also the position code and the Show row. The step headings, the
highlight on the pressed button and the verdicts are added by the script that
lays the cells out; nothing on the page itself was changed. The animation is an
APNG.
