# Obsidian Quartz Publish

## Overview

**obsidian-quartz-publish** is a command-line tool designed for seamless integration between Obsidian, a powerful note-taking and knowledge management application, and Quartz, a static site generator. This tool simplifies the process of publishing selected Obsidian notes to Quartz by automating the transfer of relevant content and embedded files.

## Features

- **Effortless Publication**: Identify notes for publishing in Obsidian by tagging them with `#publish`. obsidian-quartz-publish will recognize these tagged notes and transfer them to Quartz for further processing.

- **Smart Synchronization**: The tool intelligently checks for changes within the specified Obsidian vault. If no modifications are detected in the notes marked for publishing, obsidian-quartz-publish refrains from copying files, optimizing the publishing process.

- **Embedded File Support**: Ensure a comprehensive transfer by including all embedded files associated with the selected notes. obsidian-quartz-publish captures not only the note content but also any linked files, images, or attachments.

- **Seamless Quartz Integration**: Once the transfer is initiated, obsidian-quartz-publish seamlessly integrates with Quartz, enabling it to build the updated site with the new content.

## Prerequisites

- [Obsidian](https://obsidian.md/): Ensure that you have Obsidian installed and configured with your local vault.

- [Quartz](https://quartz.jzhao.xyz/): Set up Quartz as your static site generator, including the necessary configuration for content directories.

## Installation
Simply get the newest executable or build it yourself.

## Usage

1. Tag the notes you want to publish in Obsidian with `#publish`.

2. Run the following command to initiate the publishing process:
    ```bash
    obsidian-quartz-publish /absolute/path/to/vault /absolute/path/to/quartz_dir
    ```

3. obsidian-quartz-publish will analyze the specified Obsidian vault, identify notes marked for publishing, and transfer them to the content directory of Quartz.

4. If changes are detected, Quartz will build the updated site with the new content.

