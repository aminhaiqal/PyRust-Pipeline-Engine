import rpipeline
import time


STEP_REGISTRY = {}

def register(name, step_cls):
    STEP_REGISTRY[name] = step_cls

def get_step(name):
    return STEP_REGISTRY[name]


class Step:
    def execute(self, data):
        raise NotImplementedError
    
class PyStep(Step):
    def execute(self, data):
        return [x + 1 for x in data]

class RustStep(Step):
    def execute(self, data):
        return rpipeline.process(data)

register("py_step", PyStep)
register("rust_step", RustStep)


class Pipeline:
    def __init__(self, steps):
        self.steps = steps

    def run(self, data):
        for step in self.steps:
            start = time.time()
            data = step.execute(data)
            print(f"{step.__class__.__name__} took {time.time() - start:.4f}s")
        return data


def main():
    pipeline = Pipeline([
        get_step("py_step")(),
        get_step("rust_step")(),
    ])

    data = [2, 3]
    result = pipeline.run(data)

    print(result)


if __name__ == "__main__":
    main()
