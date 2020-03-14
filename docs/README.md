# Azuma Slate Documentation

To build this documentation, first build the docker image:

```
docker build -t slate .
```

You can then build the documentation, in this case into build/:

```
docker run --rm -v $PWD:/slate -v $PWD/build:/build slate
```

On Windows/PowerShell:

```
docker run --rm -v ${PWD}:/slate -v ${PWD}/build:/build slate
````