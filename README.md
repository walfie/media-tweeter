# media-tweeter

Made for my own very specific use case, where I do the following:

1. Generate images, where the filenames are URL-safe base-64 encoded text
2. Select a random image from a directory
3. Tweet the image, where the tweet text is the base-64 decoded filename
4. Delete the image

This project handles steps 2 to 4.

## Example

Choose a random media file from `/path/to/images`, tweet it using the
credentials in `credentials.env`, and delete the file on success:

```bash
ENV=credentials.env media-tweeter tweet --rm /path/to/images
```

This is using bash syntax for environment variables, which may differ depending
on your shell.

The `credentials.env` file has a `dotenv`-compatible format:

```
CONSUMER_KEY=insert
CONSUMER_SECRET=your
ACCESS_TOKEN=credentials
ACCESS_TOKEN_SECRET=here
```

You can also specify credentials as plain environment variables, or as
arguments (see `--help`).

