# llamada_tools

Some simple tools in *rust*, intended to be built into *WASM* and used with a *WASI* runtime as targets for LLM tool calls.  This package is written for and used by [*llamada*](https://github.com/kemenril/llamada). Tools to allow the LLM to manage files are included, as is a simple *echo* tool, and one which can be used to get the local time and date.  *JSON* schemas for all of the tools are included as well.

## Prerequisites
   * The *rust* compiler and build system
   * The target for *wasm32-wasip1*, or however else you want to build it, which is likely not included in the default set of *rust* development packages.

## Build
      cargo build --target wasm32-wasip1 
... or there's a build.sh script in the root which does that.

## Installation

If you're running these agains *llamada*, copy the *wasm* files from **target/wasm32-wasip1/release** with **schema/*.schema** into **~/llamada/tools**.  If you're not, do something else appropriate to the environment in which you're using the tools.

## What's included

   * *datetime* - Fetches a structure with system date and time information.
   * *delete_file* - Deletes a local file.  (Actually moves it into /Trash; remember these are intended to be run in a WASI sandbox, so this should not be the real path on the host system.)
   * *echo* - Returns the input it gets, verbatim.
   * *list_files* - Feeds back a directory full of files in a *JSON* list.
   * *mkdir* - Creates a directory.
   * *read_file* - Reads out the contents of a file.
   * *write_file* - Writes data into a new file.
