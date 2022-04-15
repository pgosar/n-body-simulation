cls
@pushd %~dp0
msbuild .\n-body-simulation.sln
.\x64\Debug\n-body-simulation.exe
@popd