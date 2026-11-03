import type { TTunnelKind } from './tunnels-contract';

/** `localhost:5432 to db:5432, via oracle-vps`, or `SOCKS proxy on localhost:1080, via …`. */
export function describeTunnel(kind: TTunnelKind, hostLabel: string): string {
  const via = `via ${hostLabel}`;
  return kind.type === 'local'
    ? `localhost:${String(kind.localPort)} to ${kind.targetAddress}:${String(kind.targetPort)}, ${via}`
    : `SOCKS proxy on localhost:${String(kind.localPort)}, ${via}`;
}
