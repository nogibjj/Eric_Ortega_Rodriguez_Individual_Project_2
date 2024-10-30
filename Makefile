install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

format:	
	black *.py 

lint:
	ruff check *.py mylib/*.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

# Docker
deploy:
	@echo "Deploying application..."
	docker build -t my-app .
	docker run -d -p 80:80 my-app

# running all checks 
all: install lint test format

# used for multiple tasks
pre-deploy: install refactor test

# deploy
deploy-all: pre-deploy deploy
