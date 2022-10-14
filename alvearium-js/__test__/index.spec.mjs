import test from 'ava'

import { getGlobalDynamicProperties } from '../index.js'

test('dynamic global properties', async (t) => {
  const properties = await getGlobalDynamicProperties();
  t.truthy(properties.headBlockId);
})