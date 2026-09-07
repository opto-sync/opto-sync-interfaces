import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { before, test } from 'node:test';
import { crossValidate, emitTypeSpecJsonSchema, loadSchemaCollection } from '../tmp/tsjsv/src/index.mjs';

// The 55 existing wire cases exercise the raw validator directly. These checks
// additionally exercise the differential runner, where synthesized probes used
// to hide explicit expectations that happened to carry the same JSON value.
let authoredCollection;
let generatedCollection;
before(async () => {
  const emitted = await emitTypeSpecJsonSchema({
    entry: resolve('schema-authority-canary/main.tsp'),
    outputDir: resolve('tmp/peer-wire/probe-generated'),
    bundleId: 'probe.generated.schema.json', sealObjectSchemas: true,
  });
  authoredCollection = await loadSchemaCollection(resolve('schema-authority-canary/authored.schema.json'));
  generatedCollection = await loadSchemaCollection(emitted.generatedPath);
});

for (const reverse of [false, true]) {
  for (const [instance, first, second, ruleId] of [
    ['typespec', 'accepted', 'rejected', 'corpus-instance-accepted'],
    [null, 'rejected', 'accepted', 'corpus-instance-rejected'],
  ]) {
    test(`real peer lanes retain contradictory ${JSON.stringify(instance)} expectations, reverse=${reverse}`, () => {
      // Deliberately inconsistent test inputs: one fixture must fail even though
      // the two authorities agree. This is a regression oracle, not production data.
      const expectations = reverse ? [second, first] : [first, second];
      const result = crossValidate({
        authoredCollection, generatedCollection,
        declarationMap: [{ typespec: 'OreSchemaAuthority.AuthorityKind', authored: 'AuthorityKind', generated: 'AuthorityKind' }],
        corpus: expectations.map((expectation, index) => ({
          declaration: 'AuthorityKind', instance, expectation,
          path: `AuthorityKind/expectation-${index}.json`,
          relativePath: `AuthorityKind/expectation-${index}.json`,
        })),
        maxProbes: 64,
      });
      assert.equal(result.summary.comparedDeclarations, 1, 'both real declarations must be present');
      assert.equal(result.summary.corpusInstances, 2);
      assert.equal(result.summary.refusals, 0);
      assert.equal(result.summary.divergences, 0, 'these unchanged peer contracts should agree');
      assert.equal(result.findings.length, 1, 'agreement must not erase the explicit contradictory oracle');
      assert.equal(result.findings[0].ruleId, ruleId);
      const wrongIndex = expectations.indexOf(instance === null ? 'accepted' : 'rejected');
      assert.ok(result.findings[0].message.includes(`expectation-${wrongIndex}.json`));
    });
  }
}
