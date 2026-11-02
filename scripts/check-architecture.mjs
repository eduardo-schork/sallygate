import { execFileSync } from 'node:child_process';

// The dependency rule from docs/architecture.md: each layer may only use the
// workspace crates and external crates listed here.
const ALLOWED_DEPENDENCIES = {
  'sallygate-domain': { workspace: [], external: ['thiserror', 'uuid', 'zeroize'] },
  'sallygate-application': {
    workspace: ['sallygate-domain'],
    external: ['async-trait', 'thiserror', 'uuid'],
  },
  'sallygate-infrastructure': {
    workspace: ['sallygate-domain', 'sallygate-application'],
    external: null,
  },
};

const FORBIDDEN_IN_INFRASTRUCTURE = ['tauri', 'sallygate'];

const metadata = JSON.parse(
  execFileSync('cargo', ['metadata', '--format-version', '1', '--no-deps'], { encoding: 'utf8' }),
);
const workspaceCrateNames = new Set(metadata.packages.map((cratePackage) => cratePackage.name));
const violations = [];

for (const cratePackage of metadata.packages) {
  const allowed = ALLOWED_DEPENDENCIES[cratePackage.name];

  if (!allowed) {
    continue;
  }

  const normalDependencies = cratePackage.dependencies.filter(
    (dependency) => dependency.kind === null,
  );

  for (const dependency of normalDependencies) {
    const isWorkspaceCrate = workspaceCrateNames.has(dependency.name);
    const isAllowed = isWorkspaceCrate
      ? allowed.workspace.includes(dependency.name)
      : allowed.external === null
        ? !FORBIDDEN_IN_INFRASTRUCTURE.includes(dependency.name)
        : allowed.external.includes(dependency.name);

    if (!isAllowed) {
      violations.push(`${cratePackage.name} must not depend on ${dependency.name}`);
    }
  }
}

if (violations.length > 0) {
  for (const violation of violations) {
    console.error(`Architecture violation: ${violation}. See docs/architecture.md.`);
  }

  process.exitCode = 1;
}
