#Sarateese

This is a interpreter and partial compiler that interprets my own language [Sarateese]
Sarateese is a language with simplified rust syntax that focuses on parallelism.
View the DOCS txt to see examples of Sarateese code.
To run Sarateese pass the file name as a command line parameter, optionally with -v or -vv.

What makes Sarateese special is the ability to define blocks which all run in parallel.
All programming languages so far have been designed sequentially with single core CPUs in mind with the ability to write parallel code.
Sarateese takes advantage by introducing automatic parallelism to all code through a multi-threaded dependency graph. 
