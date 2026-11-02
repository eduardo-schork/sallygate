const DESIGN_VALUE_PATTERN =
  /#[0-9a-f]{3,8}\b|(?:rgb|hsl)a?\(|(?:^|[^\w-])-?(?:\d+\.?\d*|\.\d+)(?:px|rem|em|vh|vw|vmin|vmax|%|ms|s|deg|rad|turn)\b/giu;

const UNITLESS_DESIGN_PROPERTY_PATTERN =
  /(?:font-weight|line-height|opacity|z-index)\s*:\s*-?(?:\d+\.?\d*|\.\d+)/giu;

const ALLOWED_MECHANICAL_VALUES = new Set(['100%']);

function findHardcodedValue(value) {
  for (const match of value.matchAll(DESIGN_VALUE_PATTERN)) {
    const matchedValue = match[0].trim();

    if (!ALLOWED_MECHANICAL_VALUES.has(matchedValue)) {
      return matchedValue;
    }
  }

  return value.match(UNITLESS_DESIGN_PROPERTY_PATTERN)?.[0];
}

export const NO_HARDCODED_DESIGN_VALUES = {
  meta: {
    type: 'problem',
    docs: {
      description: 'Disallow raw visual values in component implementation modules',
    },
    messages: {
      hardcoded:
        'Replace the hardcoded design value "{{ value }}" with a token from @faber-ui/tokens.',
    },
    schema: [],
  },
  create(context) {
    return {
      Literal(node) {
        if (typeof node.value !== 'string') {
          return;
        }

        const hardcodedValue = findHardcodedValue(node.value);

        if (hardcodedValue) {
          context.report({ node, messageId: 'hardcoded', data: { value: hardcodedValue } });
        }
      },
      TemplateElement(node) {
        const hardcodedValue = findHardcodedValue(node.value.raw);

        if (hardcodedValue) {
          context.report({ node, messageId: 'hardcoded', data: { value: hardcodedValue } });
        }
      },
    };
  },
};
