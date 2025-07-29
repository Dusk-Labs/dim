const ESLint = require("eslint").ESLint;

const removeIgnoredFiles = async (files) => {
  const eslint = new ESLint();
  const isIgnored = await Promise.all(
    files.map((file) => {
      return eslint.isPathIgnored(file);
    })
  );
  const filteredFiles = files.filter((_, i) => !isIgnored[i]);
  return filteredFiles.join(" ");
};

const config = {
  "src/**/*": ["prettier --write --ignore-unknown"],
  "src/**/*.{js,jsx,ts,tsx}": [
    async (files) => {
      const filesToLint = await removeIgnoredFiles(files);
      return `eslint --fix --max-warnings=0 ${filesToLint}`;
    },
    () => "tsc",
  ],
};

module.exports = config;
