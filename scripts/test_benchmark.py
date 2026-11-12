import copy
import importlib.util
from pathlib import Path
import unittest
import sys
sys.dont_write_bytecode = True

spec = importlib.util.spec_from_file_location('checker', Path(__file__).with_name('check-benchmark.py'))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


class Acceptance(unittest.TestCase):
    def setUp(self):
        self.data = dict(build_profile='release', version=1, mission_passed=True, adapter='hardware Vulkan', resolution=[1920, 1080],
                         stages={s: {'raw_ms': [10.] * 120} for s in checker.STAGES})

    def test_complete_and_optional_gpu(self):
        self.assertEqual(checker.validate(self.data), [])

    def test_instrumentation_requires_evidence_but_gpu_is_optional(self):
        self.data['instrumentation'] = True
        self.assertTrue(checker.validate(self.data))
        names = ('lib/simulate', 'main/controls', 'main/visuals', 'hud/update',
                 'presentation/rigs', 'rain/update', 'effects/combat_effects')
        paths = ['saltline/main_world_ms', 'saltline/render/submissions', 'render/main/elapsed_cpu']
        paths.extend(f'saltline/system/{name}_ms' for name in names)
        self.data['observations'] = [{'path': path} for path in paths]
        self.assertEqual(checker.validate(self.data), [])

    def test_reject_missing_invalid_and_incomplete_evidence(self):
        for key, value in [('mission_passed', False), ('adapter', 'device_type: Cpu'), ('version', 0)]:
            data = copy.deepcopy(self.data)
            data[key] = value
            self.assertTrue(checker.validate(data))
        for samples in ([], [10.] * 119, [20.] * 120, [float('nan')] * 120, ['bad'] * 120):
            data = copy.deepcopy(self.data)
            data['stages']['title']['raw_ms'] = samples
            self.assertTrue(checker.validate(data))
        del self.data['stages']['waterfront']
        self.assertTrue(checker.validate(self.data))


if __name__ == '__main__':
    unittest.main()
